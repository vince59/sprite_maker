use image::{GenericImageView, Rgba, RgbaImage};
use std::cmp::max;
use std::path::Path;

fn create_and_overlay_filmstrip(
    input_filename1: &str,
    input_filename2: &str,
    photo_width: u32,
    photo_height: u32,
    num_photos: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ouvrir la première image (celle qui va être répétée pour créer une nouvelle pellicule)
    let img1 = image::open(input_filename1)?;
    let (width1, height1) = img1.dimensions();

    // Ouvrir la seconde image (celle qui contient la pellicule)
    let img2 = image::open(input_filename2)?;
    let (width2, height2) = img2.dimensions();

    // Vérifier que la pellicule contient suffisamment de photos
    if num_photos > width2 / photo_width {
        return Err(
            "Le nombre de photos dans la pellicule est trop grand pour l'image fournie.".into(),
        );
    }

    // Créer une nouvelle image pour la pellicule finale
    let mut output_image = RgbaImage::new(width1 * num_photos, max(height1, height2));

    // Remplir la nouvelle pellicule avec la première image répétée
    for i in 0..num_photos {
        for x in 0..width1 {
            for y in 0..height1 {
                let pixel = img1.get_pixel(x % width1, y % height1);
                output_image.put_pixel(i * width1 + x, y, pixel);
            }
        }
    }

    let delta_x = if width1 < photo_width {
        0
    } else {
        (width1 - photo_width) / 2
    };

    // Superposer la pellicule de la seconde image
    for i in 0..num_photos {
        for x in 0..photo_width {
            for y in 0..photo_height {
                let pixel = img2.get_pixel(x + (i * photo_width), y);
                let pixel2 = output_image.get_pixel(i * width1 + x + delta_x, y);
                let pixel_to_use = if pixel[3] == 0 { *pixel2 } else { pixel };
                output_image.put_pixel(i * width1 + x + delta_x, y, pixel_to_use);
            }
        }
    }

    // Générer le nom du fichier de sortie
    //let output_filename = format!("{}_{}.png", input_filename1, input_filename2);
    let output_filename = input_filename1.replace(".png", input_filename2);

    // Créer un chemin à partir du nom de fichier généré
    let output_path = Path::new(&output_filename);

    // Sauvegarder l'image modifiée directement dans le fichier de sortie
    output_image.save(output_path)?;

    Ok(())
}

fn add_grid_to_image(
    input_filename: &str,
    cell_width: u32,
    cell_height: u32,
    grid_color: [u8; 4],
) -> Result<(), Box<dyn std::error::Error>> {
    // Ouvrir l'image source
    let img = image::open(input_filename)?;
    let (width, height) = img.dimensions();

    // Créer une nouvelle image de type RGBA (avec transparence)
    let mut output_image = RgbaImage::new(width, height);

    // Copier l'image source dans la nouvelle image
    for (x, y, pixel) in img.pixels() {
        output_image.put_pixel(x, y, pixel);
    }

    // Ajouter la grille
    for x in (0..width).step_by(cell_width as usize) {
        for y in (0..height).step_by(cell_height as usize) {
            // Tracer une ligne horizontale de 1 pixel de large
            for x_line in x..(x + cell_width).min(width) {
                output_image.put_pixel(x_line, y, Rgba(grid_color)); // Couleur personnalisée
            }
            // Tracer une ligne verticale de 1 pixel de large
            for y_line in y..(y + cell_height).min(height) {
                output_image.put_pixel(x, y_line, Rgba(grid_color)); // Couleur personnalisée
            }
        }
    }

    // Générer le nom du fichier de sortie
    let output_filename = input_filename.replace(".png", "_out.png"); // Remplace par le suffixe que tu souhaites

    // Créer un chemin à partir du nom de fichier généré
    let output_path = Path::new(&output_filename);

    // Sauvegarder l'image modifiée directement dans le fichier de sortie
    output_image.save(output_path)?;

    Ok(())
}

fn combine_images_vertically(
    input_filename1: &str,
    input_filename2: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ouvrir la première image
    let img1 = image::open(input_filename1)?;
    let (width1, height1) = img1.dimensions();

    // Ouvrir la deuxième image
    let img2 = image::open(input_filename2)?;
    let (width2, height2) = img2.dimensions();

    // Calculer la taille de la nouvelle image (la hauteur sera la somme des deux images)
    let output_width = std::cmp::max(width1, width2); // Largeur est la plus grande des deux
    let output_height = height1 + height2; // Hauteur est la somme des hauteurs

    // Créer une nouvelle image pour la sortie
    let mut output_image = RgbaImage::new(output_width, output_height);

    // Copier les pixels de la première image dans la nouvelle image
    for x in 0..width1 {
        for y in 0..height1 {
            let pixel = img1.get_pixel(x, y);
            output_image.put_pixel(x, y, pixel);
        }
    }

    // Copier les pixels de la deuxième image dans la nouvelle image (en dessous de la première image)
    for x in 0..width2 {
        for y in 0..height2 {
            let pixel = img2.get_pixel(x, y);
            output_image.put_pixel(x, y + height1, pixel); // Décalage vertical
        }
    }

    // Générer le nom du fichier de sortie
    let output_filename = input_filename1.replace(".png", input_filename2);

    // Créer un chemin à partir du nom de fichier généré
    let output_path = Path::new(&output_filename);

    // Sauvegarder l'image modifiée directement dans le fichier de sortie
    output_image.save(output_path)?;

    Ok(())
}

fn main() {
    // Ajout d'une grille sur une image :
    /*
    let input_filename = "fire2_64.png"; // Remplace par le chemin de ton image source
    let cell_width = 64;  // Largeur des cellules de la grille
    let cell_height = 64; // Hauteur des cellules de la grille
    let grid_color = [255, 0, 0, 255]; // Couleur de la grille (rouge ici, format RGBA)

    if let Err(e) = add_grid_to_image(input_filename, cell_width, cell_height, grid_color) {
        eprintln!("Erreur lors du traitement de l'image : {}", e);
    }*/

    // création d'une pellicule photo
    /*
    let images:Vec<&'static str>  = vec![
        "temple.png",
        "space_port.png",
        "labo.png",
        "radio.png",
        "greenhouse.png",
        "greenhouse2.png",
        "greenhouse3.png",
        "rocket.png"
    ];
    for img in images {
        let input_filename1 = img; // Première image pour la pellicule répétée
        let input_filename2 = "fire.png"; // Seconde image (la pellicule)
        let photo_width = 64; // Largeur de chaque photo
        let photo_height = 64; // Hauteur de chaque photo
        let num_photos = 10; // Nombre de photos dans la pellicule

        println!("{}",input_filename1);
        if let Err(e) = create_and_overlay_filmstrip(
            input_filename1,
            input_filename2,
            photo_width,
            photo_height,
            num_photos,
        ) {
            eprintln!("Erreur lors du traitement des images : {}", e);
        }
    }
    */
    //met la seconde image sous la première
    let images: Vec<(&'static str, &'static str)> = vec![
        ("temple.png", "templefire.png"),
        ("space_port.png", "space_portfire.png"),
        ("labo.png", "labofire.png"),
        ("radio.png", "radiofire.png"),
        ("greenhouse.png", "greenhousefire.png"),
        ("greenhouse2.png", "greenhouse2fire.png"),
        ("greenhouse3.png", "greenhouse3fire.png"),
        ("rocket.png", "rocketfire.png"),
    ];
    for (img1, img2) in images {
        let input_filename1 = img1; // Première image
        let input_filename2 = img2; // Deuxième image

        if let Err(e) = combine_images_vertically(input_filename1, input_filename2) {
            eprintln!("Erreur lors du traitement des images : {}", e);
        }
    }
}
