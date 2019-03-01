package Helper;

import java.util.Random;

public class Variants {

    private static final String[] SIZE = {
            "characteristicSelect_item_Size_52",
            "characteristicSelect_item_Size_50",
            "characteristicSelect_item_Size_48",
            "characteristicSelect_item_Size_46",
            "characteristicSelect_item_Size_44"};

    public static String getRandomSize() {
        return SIZE[new Random().nextInt(SIZE.length)];
    }

    private static final String[] MATERIAL = {
            "characteristicSelect_item_Material_Glass",
            "characteristicSelect_item_Material_Metal",
            "characteristicSelect_item_Material_Tree",
            "characteristicSelect_item_Material_Wooden",
            "characteristicSelect_item_Material_Gold"};

    public static String getRandomMaterial() {
        return MATERIAL[new  Random().nextInt(MATERIAL.length)];
    }

    private static final String[] COLOUR = {
            "characteristicSelect_item_Colour_Brown",
            "characteristicSelect_item_Colour_Red",
            "characteristicSelect_item_Colour_Black",
            "characteristicSelect_item_Colour_Blue",
            "characteristicSelect_item_Colour_White"};

    public static String getRandomColour() {
        return COLOUR[new Random().nextInt(COLOUR.length)];
    }
}

