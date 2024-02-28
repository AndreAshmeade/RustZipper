use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::fs::File;
use zip::write::FileOptions;
use zip::ZipWriter;

use std::error::Error;

use zip::ZipArchive;

pub fn create_file() -> Result<(), Box<dyn std::error::Error>> {  

//create a path for zip file 
 let path = Path::new("example.zip");
 //create file for path 
 let file = File::create(&path)?;

 //initialize the archive 
 let mut zip = ZipWriter::new(file);

 //create a file in the archive
 //start writing  its' contents. 
 zip.start_file("readme.txt", FileOptions::default())?;
 // Attemps to write an entire buffer into this writer 
 zip.write_all(b"Hello, World!\n")?;
 //Apply the changes you've made. 
 zip.finish()?;

 println!("Zip file created successfully!");

 Ok(())
}

pub fn readfile() -> Result<(), Box<dyn Error>>{
    let path = Path::new("example.zip");

    //Open the zip file for reading. 
    let file = File::open(&path)?;
    let mut archive = ZipArchive::new(file)?;

    //Iterate through all the files to in the ZIP archive.
    for i in 0..archive.len(){
        let mut file = archive.by_index(i)?;
        println!("File name: {}", file.name());
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        //Now, buffer contains the content of the file 

    }
    Ok(())
}

pub fn compress() -> Result<(), Box<dyn Error>>{
    let zip_file_path = Path::new("compressed_files.zip");
    let zip_file = File::create(&zip_file_path)?;

    let mut zip = ZipWriter::new(zip_file);

    //Define the files you want to compress. 
    let files_to_compress: Vec<PathBuf> = vec![
        PathBuf::from("exampleImage.png"),
        PathBuf::from(".gitignore")
        //Add more files needed 
    ];

    //Set compression options (e.g., compression method) 
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::DEFLATE);

    //Iterate through the files and add them to the ZIP archive. 
    for file_path in &files_to_compress {
        
        let file = File::open(file_path)?;
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        //Adding the file to the ZIP archive. 
        zip.start_file(file_name, options)?;

        let mut buffer = Vec::new();
        io::copy(&mut file.take(u64::MAX), &mut buffer)?;

        zip.write_all(&buffer)?;
    }

    zip.finish()?;

    println!("Files compressed successfully to {:?}", zip_file_path);

    Ok(())
}


pub fn extract() -> Result<(), Box<dyn Error>>{
 let zip_file_path = Path::new("compressed_files.zip"); 
 let zip_file = File::open(zip_file_path)?;

 let mut archive = ZipArchive::new(zip_file)?; 
 let extraction_dir = Path::new("extracted_files");

 //Create the directory if it does not exist. 
 if !extraction_dir.exists(){
     std::fs::create_dir(extraction_dir)?;
 }

 //Iterate through the files in the ZIP archive. 
 for i in 0..archive.len(){
     let mut file = archive.by_index(i)?;
     let file_name = file.name().to_owned();

     //Create the path to the extracted file in the destination directory. 
     let target_path = extraction_dir.join(file_name);

     //Create the destination directory if it does not exist. 
     if let Some(parent_dir) = target_path.parent(){
         std::fs::create_dir_all(parent_dir)?;
     }

     let mut output_file = File::create(&target_path)?;

     //Read the contents of the file from the ZIP archive and write them to 
     io::copy(&mut file, &mut output_file)?;
 }


 println!("Files successfully extracted to {:?}", extraction_dir);


 Ok(())

}



