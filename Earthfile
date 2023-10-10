VERSION 0.7

deps:
    FROM python:3.11-buster
    RUN pip install poetry

build:
    FROM +deps
    WORKDIR /code
    COPY --dir mergefast tests .
    COPY build.py poetry.lock pyproject.toml README.md .
    RUN python3 build.py build_ext --inplace

test:
    FROM +build
    RUN poetry install
    RUN poetry run python tests/test.py 

perf:
    FROM +build
    RUN poetry install
    RUN poetry run python tests/perf.py 
    SAVE ARTIFACT perf.png AS LOCAL perf.png

reformat-c:
    FROM alpine:3.13
    RUN apk add clang
    WORKDIR /code
    COPY merge-fast/.clang-format .
    COPY merge-fast/*.h merge-fast/*.c .
    RUN which clang-format # test that clang-format exists, since find won't bubble up exec errors
    RUN find -regex '.*.\(c\|h\)$' -exec clang-format -i {} \;
    SAVE ARTIFACT *.h AS LOCAL ./merge_fast/
    SAVE ARTIFACT *.c AS LOCAL ./merge_fast/
