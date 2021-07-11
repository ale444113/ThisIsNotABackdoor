package tcpConnection

//simple tcp echo server
import (
	"bufio"
	"fmt"
	"net"
	"os"
)

func handleConnection(conn net.Conn) {
	line, err := readLine(conn)
	if err != nil {
		fmt.Println(err)
		return
	}
	//echo it back
	fmt.Println(line)
}

func sendCommands(conn net.Conn) {
	fmt.Print("> ")
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan() // use `for scanner.Scan()` to keep reading
	command := scanner.Text()
	conn.Write([]byte(command))
}
