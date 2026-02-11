typedef struct Details {
struct inner * inner;
} Details;

struct inner{
int* secret_key;
};


struct Student<U> {
U uniqid;
char ** name;
Details *ds;
};

struct Employee{
int eid;
char ** name;
};

