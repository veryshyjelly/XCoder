import {invoke} from '@tauri-apps/api/tauri'
import {notifications} from '@mantine/notifications'
import {IconX} from "@tabler/icons-react";

export const set_directory = async (directory: string) => {
    try {
        await invoke("set_directory", {directory: directory});
        return true;
    } catch (e) {
        console.log(e);
        notifications.show({
            id: "directory_not_set",
            title: "Directory not found",
            message: "The specified directory was not found",
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        })
        return false;
    }
}

export const get_directory = async () => {
    try {
        return await invoke("get_directory");
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_get_directory",
            message: "Cannot get the directory",
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        })
        return "";
    }
}

export const set_contest_type = async (contest_type: string) => {
    try {
        await invoke("set_contest_type", {contestType: contest_type});
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_set_contest_type",
            message: e as string,
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        })
        return false;
    }
}

export const set_problem_type = async (problem_types: string[]) => {
    try {
        await invoke("set_problem_type", {problemTypes: problem_types});
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_set_problem_type",
            message: e as string,
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        })
        return false;
    }
}

export const set_language = async (language: string) => {
    try {
        await invoke("set_language", {language: language});
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_set_language",
            message: e as string,
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        });
        return false;
    }
}

export const set_show_solved = async (show_solved: boolean) => {
    try {
        await invoke("set_show_solved", {showSolved: show_solved});
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_set_show_solved",
            message: e as string,
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        });
        return false;
    }
}

export const next = async () => {
    try {
        await invoke("next");
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_next",
            message: e as string,
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        });
        return false;
    }
}

export const previous = async () => {
    try {
        await invoke("previous");
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_previous",
            message: e as string,
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        });
        return false;
    }
}

export const get_problem = async () => {
    try {
        return await invoke("get_problem") as {
            contest_id: number,
            contest_type: string,
            description: string,
            memory_limit: number,
            problem_id: string,
            test_cases_link: string,
            time_limit: number,
            title: string
        };
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_get_problem",
            message: e as string,
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        });
        return null;
    }
}

export const update_problems_list = async () => {
    try {
        await invoke("update_problems_list");
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_update_problems_list",
            message: e as string,
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        });
        return false;
    }
}

export const run = async () => {
    try {
        return await invoke("run") as {
            input: string,
            output: string,
            answer: string,
            status: string,
            time: string,
            memory: number
        };
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_run",
            message: e as string,
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        });
        return null;
    }
}

export const submit = async () => {
    try {
        return await invoke("submit") as {
            input: string,
            output: string,
            answer: string,
            status: string,
            time: string,
            memory: number
        };
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_submit",
            message: e as string,
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        });
        return null;
    }
}

export const create_file = async () => {
    try {
        await invoke("create_file");
        return true
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_create_file",
            message: e as string,
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        });
        return false;
    }
}

export const save_state = async () => {
    try {
        await invoke("save_state");
        return true
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_save_state",
            message: e as string,
            icon: <IconX size="1.1rem"/>,
            color: "red",
            autoClose: 1000,
        });
        return false;
    }
}