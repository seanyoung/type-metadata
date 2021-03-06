// Copyright 2019
//     by  Centrality Investments Ltd.
//     and Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Returns `true` if the given string is a proper Rust identifier.
pub fn is_rust_identifier(s: &str) -> bool {
	// Only ascii encoding is allowed.
	// Note: Maybe this check is superseeded by the `head` and `tail` check.
	if !s.is_ascii() {
		return false;
	}
	if let Some((&head, tail)) = s.as_bytes().split_first() {
		// Check if head and tail make up a proper Rust identifier.
		let head_ok = head == b'_' || head >= b'a' && head <= b'z' || head >= b'A' && head <= b'Z';
		let tail_ok = tail
			.iter()
			.all(|&ch| ch == b'_' || ch >= b'a' && ch <= b'z' || ch >= b'A' && ch <= b'Z' || ch >= b'0' && ch <= b'9');
		head_ok && tail_ok
	} else {
		// String is empty and thus not a valid Rust identifier.
		false
	}
}
