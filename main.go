package main

import (
	"bytes"
	"fmt"
	"log"
	"os/exec"
)

func main() {
	fmt.Println("topclean")
	apps := getApps()
	for i := 0; i < len(apps); i++ {
		clean(apps[i])
	}
	fmt.Println("done")
}

type App struct {
	name string
	cmd  string
	args []string
}

func getApps() []App {
	apps := []App{
		{"Scoop", "scoop", []string{"cleanup", "*"}},
	}
	return apps
}

func clean(app App) {
	log.Println("Cleaning " + app.name)
	var stdout, stderr bytes.Buffer
	cmd := exec.Command(app.cmd, app.args...)
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	err := cmd.Run()
	catch(err, stdout.String(), stderr.String())
}

func catch(err error, stdout string, stderr string) {
	if stdout != "" {
		log.Println(stdout)
	}
	if err != nil {
		log.Println(err)
		if stderr != "" {
			log.Println(stderr)
		}
		log.Fatalln("Exiting topclean because of a fatal error. Check the logging output above.")
	}
}
