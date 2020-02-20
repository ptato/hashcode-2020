class Library:
    def __init__(self, id, books_count, signup_duration, books_per_day, books):
        self.id = id
        self.books_count = books_count
        self.signup_duration = signup_duration
        self.books_per_day = books_per_day
        self.books = books

        self.day_starting = 0
        self.submitted_books = 0


[ books_count, libraries_count, days ] = [ int(t) for t in input().split() ]
book_scores = [ int(t) for t in input().split() ]
libraries = []
for library_index in range(libraries_count):
    [ bc, sd, bpd ] = [ int(t) for t in input().split() ]
    books = [ int(t) for t in input().split() ]
    libraries.append(Library(library_index, bc, sd, bpd, books))

count_libraries = 0
result = ""
current_day = 0
for library in sorted(libraries, key=lambda l: l.books_per_day, reverse=True):
    current_day += library.signup_duration
    if current_day > days:
        break
    n_books = (days - current_day) * library.books_per_day
    n_books = n_books if n_books <= library.books_count else library.books_count
    result += f"{library.id} {n_books}\n"
    sorted_books = sorted(library.books, reverse=True)
    result += ' '.join(map(str, sorted_books[:n_books])) + "\n"
    count_libraries += 1

print(count_libraries)
print(result)

