package main

import (
	"bytes"
	"fmt"
	"os/exec"
)

func main() {
	fmt.Println("[topclean] Starting!")
	apps := getApps()
	for i := 0; i < len(apps); i++ {
		app := apps[i]
		err := clean(app)
		catch(err)
	}
	fmt.Println("[topclean] Done!")
}

type App struct {
	name string
	cmd  string
	args []string
}

func getApps() []App {
	apps := []App{
		{"scoop", "scoop", []string{"cleanup", "*"}},
		{"npm", "npm", []string{"cache", "clean", "--force"}},
		{"yarn", "yarn", []string{"cache", "clean"}},
		{"cleanmgr", "cleanmgr", []string{"/d", "c", "/verylowdisk"}},
	}
	return apps
}

func clean(app App) error {
	fmt.Println("[topclean] Cleaning " + app.name)
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
