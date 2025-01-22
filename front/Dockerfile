# Use the official Node.js image as the base image
FROM node:latest

# Set the working directory
WORKDIR /app

# Copy package.json and package-lock.json
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy the rest of the application code
COPY . .

# Build the Svelte app
RUN npm run build

# Expose the port the app runs on
EXPOSE 4173

# Command to run the app
CMD ["npm", "run", "preview", "--","--host"]