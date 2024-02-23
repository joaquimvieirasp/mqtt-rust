use paho_mqtt::Client;

fn main() {
    // Criar um cliente MQTT
    let client = Client::new("clientId").unwrap();

    // Conectar a um broker MQTT
    client.connect("tcp://localhost:1883").unwrap();

    // Publicar uma mensagem
    client.publish("topic", "Hello, world!".as_bytes()).unwrap();

    // Se inscrever em um tópico
    client.subscribe("topic", 0).unwrap();

    // Receber mensagens
    for message in client.messages() {
        println!("Received message: {}", message.payload);
    }
}
