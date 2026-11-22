CREATE UNIQUE INDEX PackageFiles_package_filename_unique
ON PackageFiles(package_id, filename);
