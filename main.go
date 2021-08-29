package main

import (
	"bytes"
	"fmt"
	"os/exec"
)

func main() {
	fmt.Print("topclean\n\n")
	apps := getApps()
	for i := 0; i < len(apps); i++ {
		app := apps[i]
		if app.aggressive == false {
			err := clean(apps[i])
			catch(err, "", "")
		}
	}
	fmt.Print("\ndone\n")
}

type App struct {
	name       string
	cmd        string
	args       []string
	aggressive bool // may delete files that will require longer redownload, regeneration or are marked for permantent deletion
}

func getApps() []App {
	apps := []App{
		{"scoop", "scoop", []string{"cleanup", "*"}, false},
		{"npm", "npm", []string{"cache", "verify"}, false},
		{"npm", "npm", []string{"cache", "clean", "--force"}, true},
		{"yarn", "yarn", []string{"cache", "clean"}, false},
		{"cleanmgr", "cleanmgr", []string{"/d", "c", "/autoclean"}, false},
		{"cleanmgr", "cleanmgr", []string{"/d", "c", "/verylowdisk"}, true},
	}
	return apps
}

func clean(app App) error {
	fmt.Println("Cleaning " + app.name)
	var stdout, stderr bytes.Buffer
	cmd := exec.Command(app.cmd, app.args...)
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	err := cmd.Run()
	catch(err, stdout.String(), stderr.String())
	return err
}

func catch(err error, stdout string, stderr string) {
	if stdout != "" {
		fmt.Println(stdout)
	}
	if err != nil {
		fmt.Println(err)
		if stderr != "" {
			fmt.Println(stderr)
		}
	}
}
