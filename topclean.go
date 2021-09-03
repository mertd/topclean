package main

import (
	"bytes"
	"fmt"
	"os/exec"
)

var prefix = "[topclean] "
var apps = []App{
	{"scoop", "scoop", []string{"cleanup", "*"}},
	{"npm", "npm", []string{"cache", "clean", "--force"}},
	{"yarn", "yarn", []string{"cache", "clean"}},
	{"cleanmgr", "cleanmgr", []string{"/d", "c", "/verylowdisk"}},
	{"brew", "brew cleanup", []string{"brew", "cleanup"}},
	{"brew cask", "brew cask cleanup", []string{"brew", "cask", "cleanup"}},
}

func main() {
	fmt.Println(prefix + "Starting!")
	for i := 0; i < len(apps); i++ {
		app := apps[i]
		err := clean(app)
		catch(err)
	}
	fmt.Println(prefix + "Done!")
}

type App struct {
	name string
	cmd  string
	args []string
}

func clean(app App) error {
	fmt.Println(prefix + "Cleaning " + app.name)
	var stdout, stderr bytes.Buffer
	cmd := exec.Command(app.cmd, app.args...)
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	err := cmd.Run()
	cmdLog(stdout.String(), stderr.String())
	return err
}

func cmdLog(stdout string, stderr string) {
	if stdout != "" {
		fmt.Println(stdout)
	}
	if stderr != "" {
		fmt.Println(stderr)
	}
}

func catch(err error) {
	if err != nil {
		fmt.Println(err)
	}
}
