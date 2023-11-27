package main

import (
	"encoding/csv"
	"fmt"
	"github.com/anaskhan96/soup"
	"github.com/go-rod/rod"
	"github.com/go-rod/rod/lib/launcher"
	"log"
	"os"
	"strconv"
	"strings"
)

type Problem struct {
	ContestType string `json:"contest_type"`
	ContestId   uint16 `json:"contest_id"`
	ProblemId   string `json:"problem_id"`
	Link        string `json:"link"`
}

func main() {
	url := launcher.New().Headless(true).MustLaunch()
	browser := rod.New().ControlURL(url).Trace(false).MustConnect()

	page := browser.MustPage("https://www.dropbox.com/sh/nx3tnilzqz7df8a/AAAYlTq2tiEHl5hsESw6-yfLa?dl=0").MustWaitLoad()

	page.WaitDOMStable(1, 99.0)
	page.WaitElementsMoreThan("div.dig-Table-row", 485)
	//page.Mouse.Scroll(0, 20000, 200)

	doc := soup.HTMLParse(page.MustHTML())
	rows := doc.Find("div", "class", "dig-Table-body").FindAll("div", "class", "dig-Table-row")

	fmt.Println("rows = ", len(rows))

	var contestLinks = make(map[string]string)

	for _, row := range rows {
		contestLinks[row.Find("a").FullText()] = row.Find("a").Attrs()["href"]
	}

	var problemset = make([]Problem, 0)

	file, err := os.Open("problems.csv")
	if err != nil {
		panic(err)
	}
	records, err := csv.NewReader(file).ReadAll()

	linked := make(map[string]bool)

	for _, record := range records[1:] {
		linked[record[0]+record[1]] = true
		contestId, _ := strconv.ParseUint(record[1], 10, 16)
		problemset = append(problemset, Problem{
			ContestType: record[0],
			ContestId:   uint16(contestId),
			ProblemId:   record[2],
			Link:        record[3],
		})
	}

	for contest, url := range contestLinks {

		if len(contest) < 3 {
			continue
		}

		contest = strings.ToLower(contest)
		var contestType string
		switch contest[0:3] {
		case "abc":
			contestType = "abc"
		case "arc":
			contestType = "arc"
		case "agc":
			contestType = "agc"
		case "ahc":
			contestType = "ahc"
		default:
			continue
		}

		contestId, err := strconv.ParseUint(contest[3:], 10, 16)
		if err != nil {
			log.Println(err)
			continue
		}

		if _, ok := linked[contestType+strconv.Itoa(int(contestId))]; ok {
			continue
		}

		page.MustNavigate(url).WaitElementsMoreThan("div", 5)
		doc := soup.HTMLParse(page.MustHTML())
		rows := doc.FindAll("div", "class", "dig-Table-row")
		for _, row := range rows[1:] {
			problemset = append(problemset, Problem{
				ContestType: contestType,
				ContestId:   uint16(contestId),
				ProblemId:   row.Find("a").FullText(),
				Link:        row.Find("a").Attrs()["href"],
			})
		}
		fmt.Println(contest)

		//break
	}

	//fmt.Println(problemset)

	file, err = os.Create("problems.csv")
	if err != nil {
		panic(err)
	}

	writer := csv.NewWriter(file)
	writer.Write([]string{"contest_type", "contest_id", "problem_id", "link"})

	for _, problem := range problemset {
		writer.Write([]string{problem.ContestType, strconv.Itoa(int(problem.ContestId)), problem.ProblemId, problem.Link})
	}

	writer.Flush()
}