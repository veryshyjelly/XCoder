package main

import (
	"encoding/csv"
	"fmt"
	"github.com/go-rod/rod"
	"github.com/go-rod/rod/lib/launcher"
	"log"
	"os"
	"strconv"
	"strings"

	"github.com/anaskhan96/soup"
)

type Problem struct {
	ContestType string `json:"contest_type"`
	ContestId   uint16 `json:"contest_id"`
	ProblemId   string `json:"problem_id"`
	Link        string `json:"link"`
}

func main() {
	url := launcher.New().Headless(false).MustLaunch()
	browser := rod.New().ControlURL(url).Trace(true).MustConnect()

	page := browser.MustPage("https://www.dropbox.com/sh/nx3tnilzqz7df8a/AAAYlTq2tiEHl5hsESw6-yfLa?dl=0").MustWaitLoad()

	page.WaitDOMStable(1, 1.0)

	doc := soup.HTMLParse(page.MustHTML())
	rows := doc.Find("div", "class", "dig-Table-body").FindAll("div", "class", "dig-Table-row")

	fmt.Println(len(rows))

	var contestLinks = make(map[string]string)

	for _, row := range rows {
		contestLinks[row.Find("a").FullText()] = row.Find("a").Attrs()["href"]
	}

	var problemset = make([]Problem, 0)

	for contest, url := range contestLinks {
		contest = strings.ToLower(contest)
		var contest_type string
		switch contest[0:3] {
		case "abc":
			contest_type = "abc"
		case "arc":
			contest_type = "arc"
		case "agc":
			contest_type = "agc"
		case "ahc":
			contest_type = "ahc"
		default:
			continue
		}

		contestid, err := strconv.ParseUint(contest[3:], 10, 16)
		if err != nil {
			log.Println(err)
			continue
		}

		page.MustNavigate(url).MustWaitLoad().WaitDOMStable(1, 1.0)
		doc := soup.HTMLParse(page.MustHTML())
		rows := doc.Find("div", "class", "dig-Table-body").FindAll("div", "class", "dig-Table-row")
		for _, row := range rows {
			problemset = append(problemset, Problem{
				ContestType: contest_type,
				ContestId:   uint16(contestid),
				ProblemId:   row.Find("a").FullText(),
				Link:        row.Find("a").Attrs()["href"],
			})
		}

		break
	}

	fmt.Println(problemset)

	file, err := os.Create("problems.csv")
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
