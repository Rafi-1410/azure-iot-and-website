<?php
// Konfigurasi koneksi database
$host = "localhost";
$user = "root"; // Username default XAMPP
$pass = ""; // Password default kosong
$db   = "esp32data"; // Ganti dengan nama database kamu

// Membuat koneksi
$conn = mysqli_connect($host, $user, $pass, $db);

// Cek koneksi database
if (!$conn) {
    die("Koneksi database gagal: " . mysqli_connect_error());
}
?>
<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SMART FARMING PEMBERIAN PAKAN AYAM OTOMATIS BERBASIS CLOUD</title>
    <link rel="stylesheet" type="text/css" href="style1.css">
    <link rel="preconnect" href="https://fonts.gstatic.com">
    <link href="https://fonts.googleapis.com/css2?family=Poppins:wght@100;200;300;400;600;700&display=swap" rel="stylesheet">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@fortawesome/fontawesome-free@6.2.1/css/fontawesome.min.css">
</head>
<body>
    <section class="header">
        <nav>
            <a href="index1.php"></a>
            <div class="nav-links" id="navLinks">
                <i class="fa fa-times" onclick="hideMenu()"></i>
                <ul>
                    <li><a href="index.php">HOME</a></li>
                    <li><a href="background.php">BACKGROUND OF THE PROBLEM</a></li>
                    <li><a href="components.php">COMPONENTS</a></li>
                    <li><a href="benefit.php">BENEFIT</a></li>
                </ul>
            </div>
            <i class="fa fa-bars" onclick="showMenu()"></i>
        </nav>
        <div class="text-box my-element">
            <h1>Smart Farming Pemberian Pakan Ayam Otomatis Berbasis Cloud</h1>
            <p>If you want to view the sensor data in real-time, please click on the report section below</p>
            <a href="report.php" class="hero-btn">Report</a>
        </div>
    </section>

    <!-- JavaScript for Toggle Menu -->
    <script>
        var navLinks = document.getElementById("navLinks");

        function showMenu() {
            navLinks.style.right = "0";
        }
        function hideMenu() {
            navLinks.style.right = "-200px";
        }
    </script>

    <!-- Pengertian -->
    <section class="smart">
        <h1>What is Smart Farming?</h1>
        <div class="row">
            <div class="smart-col">
                <img src="images/smart.jpg">
                <p>Smart farming adalah konsep pertanian yang menggunakan teknologi modern, termasuk Internet of Things (IoT), 
                    kecerdasan buatan (AI), dan analitik data, untuk meningkatkan efisiensi, kualitas, dan 
                    kuantitas produksi pertanian.</p>
            </div>
            <div class="smart-col">
                <img src="images/iott.jpg">
                <p>Dalam praktiknya, perangkat IoT mengumpulkan data dari sensor yang ditempatkan di ladang...</p>
            </div>
            <div class="smart-col">
                <img src="images/farm.jpg">
                <p>Menurut penelitian, adopsi smart farming dapat membantu menghadapi tantangan global...</p>
            </div>
        </div>
    </section>

    <!-- Informasi -->
    <section class="informasi">
        <h2>How the System Work?</h2>
        <div class="row">
            <div class="informasi-col">
                <h1>Sistem Kerja</h1>
                <p>Sistem IoT terdiri dari sensor DHT22, HC-SR04, ESP32, dan platform Microsoft Azure...</p>
            </div>
            <div class="informasi-col">
                <img src="images/Cuplikanberita.jpg" alt="">
            </div>  
        </div>
    </section>
</body>
</html>
