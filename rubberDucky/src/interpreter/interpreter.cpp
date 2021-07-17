#include "interpreter.hpp"
#include "string.h"
void execute(String duckyScript)
{

  char input[duckyScript.length() - 1];
  strcpy(input, duckyScript.c_str());
  
  char *commands = strtok(input, "\n");
  free(input);
  while (commands != NULL)
  {

    char command[duckyScript.length() + 1];
    char argument[duckyScript.length() + 1];
    sscanf(commands, "%s %s\n", command, argument);

    if (command == "DELAY")
    {
      delay(atoi(argument));
    }else if()


    commands = strtok(NULL, "\n");
  }
}
    /*
voy a usar un elif debido a que me esta regreando este error
```
expression must have integral or enum type
```
    switch (String(argument))
    {
    case 13:
      break;
    
    default:
      break;
    }*/
