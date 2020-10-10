
#[link(name = "uhd")]
extern {

	// UHD_API uhd_error uhd_string_vector_make(uhd_string_vector_handle *h)
	pub fn uhd_string_vector_make(h:&mut usize) -> isize;

	// UHD_API uhd_error uhd_string_vector_free(uhd_string_vector_handle *h)
	pub fn uhd_string_vector_free(h:&mut usize) -> isize;

	// UHD_API uhd_error uhd_string_vector_push_back(uhd_string_vector_handle *h, const char* value)
	// UHD_API uhd_error uhd_string_vector_at(uhd_string_vector_handle h, size_t index, char* value_out, size_t strbuffer_len)
	// UHD_API uhd_error uhd_string_vector_size(uhd_string_vector_handle h, size_t *size_out)
	// UHD_API uhd_error uhd_string_vector_last_error(uhd_string_vector_handle h, char* error_out, size_t strbuffer_len)

}