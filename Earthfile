VERSION 0.7

clean:
   LOCALLY
    RUN rm -rf build dist *.egg-info 
    RUN rm ./mergefast/*.so || true

deps:
    FROM python:3.11-buster
    RUN pip install poetry

build:
    FROM +deps
    WORKDIR /code
    COPY --dir mergefast tests .
    COPY setup.py poetry.lock pyproject.toml README.md MANIFEST.in .

    # works, but not using poetry
    RUN python3 setup.py sdist

    # doesn't produce a sdist that works
    # RUN poetry install
    # RUN poetry build
    SAVE ARTIFACT ./dist AS LOCAL .

# Not working
test-direct:
    FROM +build
    RUN poetry install
    RUN poetry run python tests/test.py 

test-dist-install:
    FROM python:3.11-buster
    COPY +build/dist dist
    ENV TARFILE=$(ls ./dist/*.tar.gz)
    RUN pip install "$TARFILE"
    COPY tests .
    RUN python test.py

# Hopefully will work once we publish fixed version to pypi
test-pypi-install:
    FROM python:3.11-buster
    RUN pip install "mergefast"
    COPY tests .
    RUN python test.py

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
