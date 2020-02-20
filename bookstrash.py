import sys

class Library:
    def __init__(self, id, books_count, signup_duration, books_per_day, books):
        self.id = id
        self.signup_duration = signup_duration
        self.books_per_day = books_per_day
        self.books = books

        self.starts_on_day = 0
        self.submitted_books = set()


[ books_count, libraries_count, days ] = [ int(t) for t in input().split() ]
book_scores = { i: int(t) for i, t in enumerate(input().split()) }
libraries = []
for library_index in range(libraries_count):
    [ bc, sd, bpd ] = [ int(t) for t in input().split() ]
    books = set([ int(t) for t in input().split() ])
    libraries.append(Library(library_index, bc, sd, bpd, books))

def remove_submitted_books(libraries, books):
    for l in libraries:
        l.books = l.books - books

def piporro_points(library, current_day):
    n_books = (days - current_day) * library.books_per_day
    books_count = len(library.books)
    n_books = n_books if n_books <= books_count else books_count
    piporro = sum(sorted(library.books, key=lambda b: book_scores[b], reverse=True)[:n_books])
    return piporro


submitting_with_libraries = [ ]

result = dict()

count_libraries = 0
current_day = 0
while current_day <= days:

    try:
        library = sorted(libraries, key=lambda l: piporro_points(l, current_day), reverse=True)[0]
    except:
        break

    current_day += library.signup_duration
    if current_day > days:
        break

    libraries.remove(library)

    submitted_books = set()
    for swli, swl in enumerate(submitting_with_libraries):
        n_books = swl.books_per_day * library.signup_duration
        if n_books > len(swl.books): n_books = len(swl.books)
        sorted_books = set(sorted(swl.books, key=lambda b: book_scores[b], reverse=True)[:n_books])
        submitted_books = submitted_books.union(sorted_books)
        remove_submitted_books(submitting_with_libraries[:swli+1], submitted_books)
        swl.submitted_books = swl.submitted_books.union(sorted_books)

    library.starts_on_day = current_day - library.signup_duration
    submitting_with_libraries.append(library)

    result[library.id] = library

    remove_submitted_books(libraries, set(submitted_books))

    count_libraries += 1

result = { k:v for k, v in result.items() if len(v.submitted_books) > 0 }
print(len(result))
for id, library in result.items():
    if len(library.submitted_books) > 0:
        print(f'{id} {len(library.submitted_books)}')
        print(' '.join(map(str, library.submitted_books)))

