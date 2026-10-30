package gen

import "testing"

func TestSnakeCase(t *testing.T) {
	for in, want := range map[string]string{
		"Hello": "hello", "HelloWorld": "hello_world", "SafeAddTwo": "safe_add_two",
		"UserID": "user_id", "ToMuxedID": "to_muxed_id", "A": "a", "a": "a",
		"fnName": "fn_name", "I128Math": "i128_math", "U256Math": "u256_math",
		"Fn1": "fn1", "_voidArg": "_void_arg", "HTTPServer": "http_server",
	} {
		if got := snakeCase(in); got != want {
			t.Errorf("snakeCase(%q) = %q want %q", in, got, want)
		}
	}
}

func TestCrateName(t *testing.T) {
	if got := crateName("hello-world.v2"); got != "hello_world_v2" {
		t.Fatal(got)
	}
}
