/* Remove the alternative_names column from the artists table */
alter table public.artists
	drop column alt_names;