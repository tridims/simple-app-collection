package routes

import (
	"github.com/gorilla/mux"
	"github.com/tridims/simple-app/bookstore-apis/pkg/controllers"
)

var RegisterBookStoreRoutes = func(router *mux.Router) {
	router.HandleFunc("/api/v1/books", controllers.CreateBook).Methods("POST")
	router.HandleFunc("/api/v1/books", controllers.GetBook).Methods("GET")
	router.HandleFunc("/api/v1/books/{bookId}", controllers.GetBookById).Methods("GET")
	router.HandleFunc("/api/v1/books/{bookId}", controllers.UpdateBook).Methods("PUT")
	router.HandleFunc("/api/v1/books/{bookId}", controllers.DeleteBook).Methods("DELETE")
}
