#include <winsock2.h>
#include <iostream>

struct CLIENT_INFO
{
    SOCKET hClientSocket ;
    struct sockaddr_in clientAddr ;
};

char szServerIPAddr[] = "127.0.0.1";
int nServerPort = 5050;        

bool InitWinSock2_0() ;
BOOL WINAPI ClientThread(LPVOID lpData) ;

char separator[] = "------------------------------------------";
char separator2[] = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";

using namespace std;

int main( )
{
    if (!InitWinSock2_0())
    {
		cout << separator << endl;
        cout << "Unable to Initialize Windows Socket environment" << WSAGetLastError() << endl;
		cout << separator << endl;
        return -1;
    }

    SOCKET hServerSocket;
    hServerSocket = socket(
        AF_INET,        // Addres family. AF_INET specifies TCP/IP
        SOCK_STREAM,    // Protocol type. SOCK_STREM specified TCP
        0               // Protoco Name. Should be 0 for AF_INET address family
        );

    if (hServerSocket == INVALID_SOCKET)
    {
		cout << separator << endl;
        cout << "Unable to create Server socket" << endl;
		cout << separator << endl;
        WSACleanup(); // It cleans the environment initialized by WSAStartup()
        return -1 ;
    }

    struct sockaddr_in serverAddr;

    serverAddr.sin_family = AF_INET;     
	serverAddr.sin_addr.s_addr = inet_addr(szServerIPAddr);
    serverAddr.sin_port = htons(nServerPort);

    if (bind(hServerSocket, (struct sockaddr *) &serverAddr, sizeof(serverAddr)) == SOCKET_ERROR)
    {
		cout << separator << endl;
        cout << "Unable to bind to " << szServerIPAddr << " at port " << nServerPort << " (Server alredy running at that port?)" << endl ;
		cout << separator << endl;
        closesocket(hServerSocket) ;
        WSACleanup() ;
        return -1 ;
    }

    if (listen(hServerSocket, SOMAXCONN) == SOCKET_ERROR)
    {
		cout << separator << endl;
        cout << "Unable to put server in listen state" << endl;
		cout << separator << endl;
        closesocket( hServerSocket );
        WSACleanup();
        return -1 ;
    }

    SOCKET hClientSocket = INVALID_SOCKET;

    while (hClientSocket == INVALID_SOCKET)
    {
        struct sockaddr_in clientAddr;
        int nSize = sizeof(clientAddr);
		
        cout << separator2 << endl;
        cout << "Waiting for clients..." << endl;
        cout << separator2 << endl;

        hClientSocket = accept(hServerSocket, (struct sockaddr *) &clientAddr, &nSize);
        if (hClientSocket == INVALID_SOCKET)
        {
            cout << "Couldn't accept some client." << endl;
        }
        else
        {
            HANDLE hClientThread;
            struct CLIENT_INFO clientInfo;
            DWORD dwThreadId;

            clientInfo.clientAddr = clientAddr;
            clientInfo.hClientSocket = hClientSocket;

            system("cls");

			cout << separator2 << endl;
            cout << "Client connected from " << inet_ntoa(clientAddr.sin_addr) << endl;
			cout << separator2 << endl;

            // Start the client thread
            hClientThread = CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE) ClientThread, (LPVOID) &clientInfo, 0, &dwThreadId);
            if ( hClientThread == NULL )
            {
				cout << separator << endl;
                cout << "Unable to create client thread" << endl ;
				cout << separator << endl;
            }

            else
            {
                CloseHandle(hClientThread);
            }
        }
    }

    closesocket(hServerSocket);
    WSACleanup();
    return 0;
}

bool InitWinSock2_0()
{
    WSADATA wsaData;
    WORD wVersion = MAKEWORD(2, 0);

    if (!WSAStartup(wVersion, &wsaData)) return true;
    return false;
}


BOOL WINAPI ClientThread( LPVOID lpData )
{
    CLIENT_INFO *pClientInfo = (CLIENT_INFO *) lpData;
    char szBuffer[1024] = "";
    
    while(true){
        cout << "> ";
        cin >> szBuffer;

        int nLength = strlen(szBuffer);

        int nCntSend = 0;
        char *pBuffer = szBuffer;

        if(nLength > 1024){
            cout << separator2 << endl;
            cout << "Command is longer than 1024 so it can't be send" << endl;
            cout << separator2 << endl;
            continue;
        }

        nCntSend = send( pClientInfo->hClientSocket, pBuffer, nLength, 0);
        if( nCntSend == -1 )
        {
            cout << "Error sending the data to client" << endl ;
            continue;
        }

        strupr(szBuffer) ;
        if ( strcmp( szBuffer, "EXIT" ) == 0 )
        {
            closesocket( pClientInfo -> hClientSocket );
            break;
        }
    }
    return 0;
}

