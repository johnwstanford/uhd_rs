
use libc::{c_char, size_t};

#[link(name = "uhd")]
extern {

	pub fn uhd_string_vector_make(h:&mut usize) -> isize;
	pub fn uhd_string_vector_free(h:&mut usize) -> isize;

	// uhd_error uhd_string_vector_push_back(uhd_string_vector_handle *h, const char* value)
	
	pub fn uhd_string_vector_at(h:usize, index:size_t, value_out:*const c_char, strbuffer_len:size_t) -> isize;

	// uhd_error uhd_string_vector_size(uhd_string_vector_handle h, size_t *size_out)
	pub fn uhd_string_vector_size(h:usize, size_out:&mut usize) -> isize;

	// uhd_error uhd_string_vector_last_error(uhd_string_vector_handle h, char* error_out, size_t strbuffer_len)

}