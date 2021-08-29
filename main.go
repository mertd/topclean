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
		err := clean(apps[i])
		catch(err, "", "")
	}
	fmt.Print("\ndone\n")
}

type App struct {
	name string
	cmd  string
	args []string
}

func getApps() []App {
	apps := []App{
		{"scoop", "scoop", []string{"cleanup", "*"}},
		{"npm", "npm", []string{"cache", "verify"}},
		{"yarn", "yarn", []string{"cache", "clean"}},
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
