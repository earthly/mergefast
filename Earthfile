deps:
    FROM alpine:3.13
    RUN apk add python3 python3-dev musl-dev gcc

build:
    FROM +deps
    WORKDIR /code
    COPY setup.py merge.h merge.c bind.c .
    RUN python3 setup.py build_ext --inplace

test:
    FROM +build
    COPY test.py .
    RUN --no-cache python3 test.py

reformat-c:
    FROM alpine:3.13
    RUN apk add clang
    WORKDIR /code
    COPY .clang-format .
    COPY *.h *.c .
    RUN which clang-format # test that clang-format exists, since find won't bubble up exec errors
    RUN find -regex '.*.\(c\|h\)$' -exec clang-format -i {} \;
    SAVE ARTIFACT *.h AS LOCAL ./
    SAVE ARTIFACT *.c AS LOCAL ./
