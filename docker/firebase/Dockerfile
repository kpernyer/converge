# Firebase Emulator with Java 21 and Node.js
FROM eclipse-temurin:21-jdk-jammy

# Install Node.js 20
RUN apt-get update && apt-get install -y curl \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Firebase CLI
RUN npm install -g firebase-tools

WORKDIR /app

# Copy configuration
COPY firebase.json firestore.rules ./

# Expose ports
EXPOSE 4000 8080 9099

# Start emulators
CMD ["firebase", "emulators:start", "--only", "firestore", "--project", "demo-converge"]
