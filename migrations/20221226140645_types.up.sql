--- Section for external site types ---

/* Create enum for which type the link is for */
create type external_site_type as enum('AppleMusic','Youtube','Spotify');

/* Create a enum for what type of link it is, for example (Artist) */
create type external_type as enum('Artist','Album','Song');

/* The external site type that stores all the info needed to track sites */
create type external_site as (site_type external_site_type, site_id text, external_type external_type);

--- Section for localized names ---

/* localized_name type for localized names */
create type localized_name as (native text, romanized text, english text);

--- Section for release types ---

/* Create enum for release types */
create type release_type as enum('Album','Single','EP','Compilation','Soundtrack','Live','Remix','Other');
