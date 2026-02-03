use linfa::dataset;
use smartcore::ensemble::random_forest_classifier::{
    RandomForestClassifier, RandomForestClassifierParameters,
};
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::metrics::accuracy;
use smartcore::model_selection::train_test_split;

fn main() {
    // Load the built-in Iris dataset
    let iris = load_dataset();

    // Features (4 columns: sepal length, sepal width, petal length, petal width)
    let x = iris.data;           // DenseMatrix<f64>, shape (150, 4)

    // Target (species: 0 = setosa, 1 = versicolor, 2 = virginica)
    let y: Vec<u32> = iris.target.iter().map(|&v| v as u32).collect();

    // Split into train/test sets (80/20 split, stratified by default)
    let (x_train, x_test, y_train, y_test) = train_test_split(
        &x,
        &y,
        0.2,                        // test size = 20%
        true,                       // shuffle = true
        Some(42),                   // random seed for reproducibility
    );

    // Optional: print dataset sizes
    println!("Training samples: {}", x_train.shape().0);
    println!("Test samples:     {}", x_test.shape().0);

    // Random Forest parameters (Iris is easy → small model is enough)
    let params = RandomForestClassifierParameters::default()
        .with_n_trees(50)           // fewer trees needed than MNIST
        .with_m(2)                  // sqrt(4) ≈ 2 features per split
        .with_max_depth(5)          // trees usually don't need to be deep
        .with_min_samples_leaf(2);

    // Train the model
    let model = RandomForestClassifier::fit(&x_train, &y_train, params)
        .expect("Failed to train Random Forest");

    // Predict on test set
    let predictions: Vec<u32> = model.predict(&x_test).expect("Failed to predict");

    // Evaluate
    let acc = accuracy(&y_test, &predictions);
    println!("\nTest accuracy: {:.4}", acc);

    // Optional: show some example predictions
    println!("\nFirst 10 test predictions vs actual:");
    for i in 0..10.min(predictions.len()) {
        println!(
            "Predicted: {}, Actual: {}",
            predictions[i], y_test[i]
        );
    }
}