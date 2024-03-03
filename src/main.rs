extern crate gtk;
use gtk::{prelude::*, TextView};
use gtk::{Button, Window, Grid, WindowType};

fn main() {
    // Initialize GTK.
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new top-level window.
    let window = Window::new(WindowType::Toplevel);
    window.set_title("rustcalc");
    window.set_default_size(200, 140);
    
    /********************************* BUTTON CREATION ****************************************/
    // Create a vertical box.
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

    // Create a textview
    let textview: TextView = gtk::TextView::new();

    // Get the buffer associated with the text view.
    let buffer = textview.get_buffer().expect("Failed to get buffer");

    // Set the text in the buffer.
    buffer.set_text("Hello, world!");

    vbox.pack_start(&textview, false, true, 0);

    // Create a grid to organize the buttons.
    let grid = Grid::new();
    grid.set_row_spacing(0);
    grid.set_column_spacing(0);

    // Button labels for the grid.
    let button_labels = vec![
        "1", "2", "3", "/",
        "4", "5", "6", "*",
        "7", "8", "9", "-",
        "E", "0", "C", "+",
    ];

    // Add buttons to the grid.
    for (i, label) in button_labels.iter().enumerate() {
        let button = Button::with_label(label);
        let row = i / 4;  // Calculate row index based on position in button_labels.
        let col = i % 4;  // Calculate column index based on position in button_labels.
        grid.attach(&button, col as i32, row as i32, 1, 1);
    }

    // Add to vbox
    vbox.pack_start(&grid, false, false, 0);

    // Add the grid to the window.
    window.add(&vbox);
    /*
    // First line
    // Create a vertical box.
    let vbox1 = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    
    // Create a button with the label "1" and add it to the vbox.
    let button1 = Button::with_label("1");
    vbox1.pack_start(&button1, false, false, 0);

    // Create a button with the label "2" and add it to the vbox.
    let button2 = Button::with_label("2");
    vbox1.pack_start(&button2, false, false, 0);

    // Create another button with the label "3" and add it to the vbox.
    let button3 = Button::with_label("3");
    vbox1.pack_start(&button3, false, false, 0);

    // Create another button with the label "/" and add it to the vbox.
    let button_div = Button::with_label("/");
    vbox1.pack_start(&button_div, false, false, 0);

    // Add the vbox to the window.
    window.add(&vbox1);
    
    // Second line
    // Create a vertical box.
    let vbox2 = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    
    // Create a button with the label "4" and add it to the vbox.
    let button4 = Button::with_label("4");
    vbox2.pack_start(&button4, false, false, 0);

    // Create a button with the label "5" and add it to the vbox2.
    let button5 = Button::with_label("5");
    vbox2.pack_start(&button5, false, false, 0);

    // Create another button with the label "6" and add it to the vbox2.
    let button6 = Button::with_label("6");
    vbox2.pack_start(&button6, false, false, 0);

    // Create another button with the label "*" and add it to the vbox2.
    let button_mul = Button::with_label("*");
    vbox2.pack_start(&button_mul, false, false, 0);

    // Add the vbox2 to the window.
    window.add(&vbox2);

    // Third line
    // Create a vertical box.
    let vbox3 = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    
    // Create a button with the label "7" and add it to the vbox3.
    let button7 = Button::with_label("7");
    vbox3.pack_start(&button7, false, false, 0);

    // Create a button with the label "8" and add it to the vbox3.
    let button8 = Button::with_label("8");
    vbox3.pack_start(&button8, false, false, 0);

    // Create another button with the label "9" and add it to the vbox3.
    let button9 = Button::with_label("9");
    vbox3.pack_start(&button9, false, false, 0);

    // Create another button with the label "-" and add it to the vbox3.
    let button_sub = Button::with_label("-");
    vbox3.pack_start(&button_sub, false, false, 0);
  
    // Add the vbox3 to the window.
    window.add(&vbox3);
    
    // Fourth line
    // Create a vertical box.
    let vbox4 = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    
    // Create a button with the label "E" and add it to the vbox4.
    let button_enter = Button::with_label("E");
    vbox4.pack_start(&button_enter, false, false, 0);

    // Create a button with the label "0" and add it to the vbox4.
    let button0 = Button::with_label("3");
    vbox4.pack_start(&button0, false, false, 0);

    // Create another button with the label "C" and add it to the vbox4.
    let button_clear = Button::with_label("C");
    vbox4.pack_start(&button_clear, false, false, 0);

    // Create another button with the label "+" and add it to the vbox4.
    let button_add = Button::with_label("+");
    vbox4.pack_start(&button_add, false, false, 0);

    // Add the vbox4 to the window.
    window.add(&vbox4);
*/
    // Connect the 'destroy' event to quit the GTK main loop.
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    /********************************* BUTTON EVENTS ****************************************/
    // Connect a callback to handle button clicks.
    /*button1.connect_clicked(|_| {
        println!("Button clicked!");
    });*/

    // Show all widgets in the window.
    window.show_all();

    // Start the GTK main event loop.
    gtk::main();
}
