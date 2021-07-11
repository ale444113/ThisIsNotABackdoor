package tcpConnection

import "net"

func readLine(conn net.Conn) (string, error) {
	var buf [1024]byte
	n, err := conn.Read(buf[:])
	if err != nil {
		return "", err
	}
	return string(buf[:n]), nil
}
