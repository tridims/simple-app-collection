from typing import List
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics.pairwise import cosine_similarity
import sys

SUPPORTED_FILE_TYPES = ['txt']


def Plagerism_Detector(files, student):
    results = set()
    def v(Text): return TfidfVectorizer().fit_transform(Text).toarray()
    def similarity(doc1, doc2): return cosine_similarity([doc1, doc2])
    vectors = list(zip(files, v(student)))

    for stud, text_vector_a in vectors:
        n_vectors = vectors.copy()
        i = n_vectors.index((stud, text_vector_a))
        del n_vectors[i]
        for stud2, vector2 in n_vectors:
            sim_score = similarity(text_vector_a, vector2)[0][1]
            stud_pair = sorted((stud, stud2))
            match_per = (stud_pair[0], stud_pair[1], sim_score)
            results.add(match_per)
    return results


student_files = sys.argv[1:]
student_notes = []


def validate_file_types(file_list: List):
    fl = []
    for file in file_list:
        if file.split('.')[-1] in SUPPORTED_FILE_TYPES:
            fl.append(file)
        else:
            print(f'{file} is not a supported file type and will be ignored')
    return fl


assert len(student_files) > 1, 'You must provide at least two files to compare'

student_files = validate_file_types(student_files)
for file in student_files:
    with open(file, "r") as f:
        student_notes.append(f.read())


results = Plagerism_Detector(student_files, student_notes)

for result in results:
    print("Result: ", result)
