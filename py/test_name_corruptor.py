# content of test_sample.py
def parse_patterns(str):
    result = []
    sequences = [seq.strip() for seq in str.split(";")]
    for seq in sequences:
        parts = [part.strip() for part in seq.split("->")]
        result.extend(zip(parts, parts[1:]))
    return result

def test_parse_single_sequence():
    assert parse_patterns("a -> b -> c") == [("a", "b"), ("b", "c")]

def test_parse_multiple_sequences():
    assert parse_patterns("a -> b -> c; d -> e -> f") == [("a", "b"), ("b", "c"), ("d", "e"), ("e", "f")]
