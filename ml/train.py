# Python script to train a moisture prediction model
import pandas as pd
from sklearn.linear_model import LinearRegression
import sqlite3

# Fetch data from SQLite
conn = sqlite3.connect('plant_data.db')
df = pd.read_sql_query("SELECT temperature, moisture, light FROM sensor_data", conn)

# Create feature matrix (X) and target vector (y)
X = df[['temperature', 'light']]  # Features
y = df['moisture']  # Target (moisture)

# Train model
model = LinearRegression()
model.fit(X, y)

# Save the model (using pickle or joblib)
import joblib
joblib.dump(model, 'moisture_predictor.pkl')
