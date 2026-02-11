typedef struct Details {
struct inner * inner;
}
Details;
struct inner{
int* secret_key;
};
struct Employee{
int eid;
char ** name;
};
struct Student_int { int uniqid; char ** name; Details *ds;};
