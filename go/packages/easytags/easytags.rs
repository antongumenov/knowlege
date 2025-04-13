//>> easytags
// to generate tags for structure

//<< install
go get github.com/betacraft/easytags

//>> usage in code
//<< write go generate comment
//go:generate easytags $GOFILE json,xml,sql
type User struct{
    Name string 
}

//<< generate
go generate