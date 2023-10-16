VERSION 0.7

clean:
   LOCALLY
    RUN rm -rf build dist *.egg-info 
    RUN rm ./mergefast/*.so

deps:
    FROM python:3.11-buster
    RUN pip install poetry

# wrong
build:
    FROM +deps
    WORKDIR /code
    COPY --dir mergefast tests .
    COPY setup.py poetry.lock pyproject.toml README.md .
    RUN python3 setup.py build_ext --inplace

test:
    FROM +build
    RUN poetry install
    RUN poetry run python tests/test.py 

#wrong?
test-from-pip:
    FROM python:3.11-buster
    RUN pip install mergefast
    RUN python -c 'import mergefast'

perf:
    FROM +build
    RUN poetry install
    RUN poetry run python tests/perf.py 
    SAVE ARTIFACT perf.png AS LOCAL perf.png

reformat-c:
    FROM alpine:3.13
    RUN apk add clang
    WORKDIR /code
    COPY mergefast/.clang-format .
    COPY mergefast/*.h mergefast/*.c .
    RUN which clang-format # test that clang-format exists, since find won't bubble up exec errors
    RUN find -regex '.*.\(c\|h\)$' -exec clang-format -i {} \;
    SAVE ARTIFACT *.h AS LOCAL ./mergefast/
    SAVE ARTIFACT *.c AS LOCAL ./mergefast/
