/* Add alternative names column to artists table */
alter table public.artists
	add column alt_names localized_name[];