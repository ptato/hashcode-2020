import sys

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
book_scores = { i: int(t) for i, t in enumerate(input().split()) }
libraries = []
for library_index in range(libraries_count):
    [ bc, sd, bpd ] = [ int(t) for t in input().split() ]
    books = set([ int(t) for t in input().split() ])
    libraries.append(Library(library_index, bc, sd, bpd, books))

def remove_submitted_books(libraries, books):
    for l in libraries:
        for n in books:
            try:
                l.books.remove(n)
            except:
                pass

def piporro_points(library, current_day):
    n_books = (days - current_day) * library.books_per_day
    books_count = len(library.books)
    n_books = n_books if n_books <= books_count else books_count
    piporro = sum(sorted(library.books, key=lambda b: book_scores[b], reverse=True)[:n_books])
    return piporro

from collections import defaultdict
amounts_of_books = defaultdict(int)
for library in libraries:
    for b in library.books:
        amounts_of_books[b] += 1


count_libraries = 0
current_day = 0
while current_day <= days:

    try:
        library = [ t for t in sorted(libraries, key=lambda l: l.signup_duration) if len(t.books) > 0 ][0]
    except:
        break

    current_day += library.signup_duration
    if current_day > days:
        break

    libraries.remove(library)

    n_books = (days - current_day) * library.books_per_day
    books_count = len(library.books)
    n_books = n_books if n_books <= books_count else books_count

    sorted_books = sorted(library.books, key=lambda b: book_scores[b], reverse=True)[:n_books]
    # sorted_books = sorted(library.books, key=lambda b: book_scores[b] / amounts_of_books[b], reverse=True)[:n_books]

    result += f"{library.id} {n_books}\n"
    result += ' '.join(map(str, sorted_books)) + "\n"

    remove_submitted_books(libraries, set(sorted_books))

    count_libraries += 1

# for library in libraries:
#     print('', file=sys.stderr)
#     print(' '.join(map(str, library.books)), file=sys.stderr)

print(count_libraries)
print(result)

