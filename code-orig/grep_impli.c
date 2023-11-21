#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void printUsage() {
    printf("Correct format is:\n");
    printf("./grep_impli.o word_to_search file_to_search\n");
    printf("./grep_impli.o -c word_to_search file_to_search\n");
}

int main(int argc, char** argv){

        //============================================================================
        // Taking in input
        //============================================================================
        if (argc < 3) {
                printUsage();
                return 1;
        }
        
        FILE* fp;
        char* line = NULL;
        size_t len = 0;
        ssize_t read;
        int i;
        //============================================================================
        // Errors in opening files
        //============================================================================
        fp = fopen(argv[argc - 1], "r");
        if(fp == NULL) {
                printf("No file found\n");
                printf("Exiting\n");
                return 1;
        }
        //============================================================================
        // If not case sensitive
        //============================================================================
        if(argc == 3) {
                while((read = getline(&line, &len, fp)) != -1) {
                        if(strstr(line, argv[1]) != NULL)
                                printf("%s", line);
                }
                fclose(fp);
                return 0;
        }
        //============================================================================
        // If case sensitive
        //============================================================================
        
        // Incorrect option passed
        if(strcmp(argv[1],"-c") != 0) {
                printf("Incorrect option: use -c\n");
                fclose(fp);
                return 1;
        }     

        // Only word is not passed
        if(2 == argc - 1) {
                if(strcmp(argv[i],"-c") != 0) {
                        printf("Incorrect option: use -c\n");
                }
                else {
                        printf("Word not entered\n");
                }
                fclose(fp);
                return 1;
        }

        // Input arguments are okay
        while((read = getline(&line, &len, fp)) != -1) {
                if(strcasestr(line, argv[2]) != NULL) {
                        printf("%s", line);
                        break;
                }
        }

        fclose(fp);
        //if(line)
                //free(line);

        return 0;
        
}