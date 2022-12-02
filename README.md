# ulid-rs for python by pyo3

pip https://pypi.org/project/ulid-rs/

ulid-rs https://github.com/dylanhart/ulid-rs

## develop & test
develop
```shell
maturin develop
```
 test
```shell
python
>>> import ulid_rs
>>> print(ulid_rs.gen_ulid_str())
01GK998PJ5NK6N8QT59EB8FFY1
>>> 


```

## build and publish
build
```shell
#maturin build
maturin build -f -r

```
publish
```shell
maturin publish
```