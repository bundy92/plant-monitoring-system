# Python script to make moisture predictions using the trained model
import joblib
import sqlite3

# Load trained model
model = joblib.load('moisture_predictor.pkl')

# Predict moisture based on current temperature and light
def predict_moisture(temperature, light):
    return model.predict([[temperature, light]])[0]

# Example usage:
print(predict_moisture(23.5, 500))
