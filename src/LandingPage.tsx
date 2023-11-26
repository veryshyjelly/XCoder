import {Box, Stack, Flex, Space, Text, Image } from "@mantine/core";

const LandingPage = () => {
    return  (
        <Flex direction={"column"} align={"center"} style={{userSelect: "none"}}>
          <Text c={"#dee0e2"} fw={500} mt={50} fz={40}>
            Welcome to XCoder
          </Text>
          <Space h={"md"}/>
          <Text c={"#6f737a"} fz={"sm"}>
            Create a new project to start from scratch or open existing folder.
          </Text>
          <Space h={"lg"}/>
          <Flex>
            <Stack m={40}>
              <Box bg={"#2b2d30"} p={15} style={{borderRadius: 7}}>
                <Image src={"/add.svg"} w={35} m={"auto"}/>
              </Box>
              <Text fz={"xs"} c={"#d5dee1"}>New Project</Text>
            </Stack>
            <Stack m={40}>
              <Box bg={"#2b2d30"} p={15} style={{borderRadius: 7}}>
                <Image src={"/folder.svg"} w={35} m={"auto"}/>
              </Box>
              <Text fz={"xs"} c={"#d5dee1"}>Open Existing</Text>
            </Stack>
          </Flex>
        </Flex>
    );
}

export default LandingPage;