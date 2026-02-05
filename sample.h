
#include <algorithm>
typedef struct Fullname{
char * first;
char * last;
} Fullname;


struct Student {
Fullname name;
};

typedef struct Teacher<T> {
T pid;
char * name;
} Teacher;

struct Employee<U>{
U empid;
float z;
};


int z;
