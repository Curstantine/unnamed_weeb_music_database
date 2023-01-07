/* To revert this migration, we need to drop the new value from the enum, but
 * we can't do that because dropping values isn't supported in Postgres. So
 * instead, we'll just drop the entire enum and recreate it without the new
 * value. To do that, we first rename the enum to a temporary name, then
 * recreate the enum without the new value, then update the columns to use
 * the new enum, then drop the temporary enum.
 */
ALTER TYPE external_site_type RENAME TO external_site_type_old;
CREATE TYPE external_site_type AS ENUM (
	'AppleMusic',
	'YouTube',
	'Spotify',
);
ALTER TABLE external_sites ALTER COLUMN type TYPE external_site_type USING type::text::external_site_type;
DROP TYPE external_site_type_old;
