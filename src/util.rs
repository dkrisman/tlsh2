const TOPVAL: [u32; 170] = [
    1, 2, 3, 5, 7, 11, 17, 25, 38, 57, 86, 129, 194, 291, 437, 656, 854, 1110, 1443, 1876, 2439,
    3171, 3475, 3823, 4205, 4626, 5088, 5597, 6157, 6772, 7450, 8195, 9014, 9916, 10907, 11998,
    13198, 14518, 15970, 17567, 19323, 21256, 23382, 25720, 28292, 31121, 34233, 37656, 41422,
    45564, 50121, 55133, 60646, 66711, 73382, 80721, 88793, 97672, 107439, 118183, 130002, 143002,
    157302, 173032, 190335, 209369, 230306, 253337, 278670, 306538, 337191, 370911, 408002, 448802,
    493682, 543050, 597356, 657091, 722800, 795081, 874589, 962048, 1058252, 1164078, 1280486,
    1408534, 1549388, 1704327, 1874759, 2062236, 2268459, 2495305, 2744836, 3019320, 3321252,
    3653374, 4018711, 4420582, 4862641, 5348905, 5883796, 6472176, 7119394, 7831333, 8614467,
    9475909, 10423501, 11465851, 12612437, 13873681, 15261050, 16787154, 18465870, 20312458,
    22343706, 24578077, 27035886, 29739474, 32713425, 35984770, 39583245, 43541573, 47895730,
    52685306, 57953837, 63749221, 70124148, 77136564, 84850228, 93335252, 102668779, 112935659,
    124229227, 136652151, 150317384, 165349128, 181884040, 200072456, 220079703, 242087671,
    266296456, 292926096, 322218735, 354440623, 389884688, 428873168, 471760495, 518936559,
    570830240, 627913311, 690704607, 759775136, 835752671, 919327967, 1011260767, 1112386880,
    1223623232, 1345985727, 1480584256, 1628642751, 1791507135, 1970657856, 2167723648, 2384496256,
    2622945920, 2885240448, 3173764736, 3491141248, 3840255616, 4224281216,
];

pub fn l_capturing(len: u32) -> u8 {
    let mut idx: u8 = 85;
    let mut bottom = 0;
    let mut top = 170;

    loop {
        if idx == 0 {
            return idx;
        }
        if (len <= TOPVAL[usize::from(idx)]) && (len > TOPVAL[usize::from(idx - 1)]) {
            return idx;
        }
        if len < TOPVAL[usize::from(idx)] {
            top = idx - 1;
        } else {
            bottom = idx + 1;
        }
        idx = (bottom + top) / 2;
    }
}

pub fn swap_byte(c: u8) -> u8 {
    let mut byte = ((c & 0xF0) >> 4) & 0x0F;
    byte |= ((c & 0x0F) << 4) & 0xF0;
    byte
}