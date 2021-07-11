package tcpConnection

//simple tcp echo server
import (
	"bufio"
	"fmt"
	"net"
	"os"
)

func handleConnection(conn net.Conn) {
	//read the first line

	for {

		line, err := readLine(conn)
		if err != nil {
			fmt.Println(err)
			return
		}
		//echo it back
		fmt.Printf("output from the client:%s\n> ", line)

	}
}

func sendCommands(conn net.Conn) {
	for {
		fmt.Print("> ")
		scanner := bufio.NewScanner(os.Stdin)
		scanner.Scan() // use `for scanner.Scan()` to keep reading
		command := scanner.Text()
		conn.Write([]byte(command))

	}
}
