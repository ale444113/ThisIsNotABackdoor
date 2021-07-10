package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
)

const (
	connHost = "localhost"
	connPort = "5050"
	connType = "tcp"
)

func printMessage(m string) {
	var separator = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
	fmt.Println(separator)
	fmt.Println(m)
	fmt.Println(separator)
}

func checkError(err error) {
	if err != nil {
		fmt.Println("Error connecting:", err.Error())
		os.Exit(1)
	}
}

func main() {
	printMessage("Server waiting connections on " + connHost + ":" + connPort + " (Connection type: " + connType + ")")
	l, err := net.Listen(connType, connHost+":"+connPort)
	if err != nil {
		fmt.Println("Error listening:", err.Error())
		os.Exit(1)
	}
	defer l.Close()

	c, err := l.Accept()
	checkError(err)
	fmt.Println("Client " + c.RemoteAddr().String() + " connected.")

	var command = ""
	for {
		fmt.Print("> ")
		scanner := bufio.NewScanner(os.Stdin)
		scanner.Scan() // use `for scanner.Scan()` to keep reading
		command = scanner.Text()
		fmt.Println(command)
		if command == "EXIT" {
			c.Write([]byte("EXIT"))
			break
		}
		if len(command) > 1024 {
			printMessage("The command is too large, it must be < 1024.")
			continue
		}
		c.Write([]byte(command))

		result, err := bufio.NewReader(c).ReadString('\n')
		checkError(err)

		fmt.Println(result)
	}
}
