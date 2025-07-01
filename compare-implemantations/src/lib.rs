pub mod anagram;
pub mod anagram_v2;

// Re-export the functions with descriptive names
pub use anagram::anagrams_for as anagrams_for_v1;
pub use anagram_v2::anagrams_for as anagrams_for_v2;

// You can also provide a default implementation
pub use anagram_v2::anagrams_for as anagrams_for;